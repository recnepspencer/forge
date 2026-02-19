//! Native egui trace viewer for Forge kernel traces.
//!
//! DOMAIN: Trace visualization — hierarchical span/decision drill-down.
//! DEPENDENCIES: `forge-core` (DecisionLog, TraceEvent), `eframe`, `notify`
//!
//! Watches a directory of JSON trace files and renders them in a native
//! window with live reload. No HTTP server, no browser.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Instant;

use eframe::egui;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::trace_store::{
    DecisionView, SpanView, TraceMeta, TraceStore,
};

/// Color palette for decision tiers.
struct TierColors;

impl TierColors {
    fn from_tier(tier: &str) -> egui::Color32 {
        match tier {
            "Deterministic" => egui::Color32::from_rgb(76, 175, 80),
            "NearBoundary" => egui::Color32::from_rgb(255, 193, 7),
            "Escalated" => egui::Color32::from_rgb(255, 152, 0),
            "PolicyApplied" => egui::Color32::from_rgb(244, 67, 54),
            _ => egui::Color32::from_rgb(158, 158, 158),
        }
    }

    fn badge_bg(tier: &str) -> egui::Color32 {
        let c = Self::from_tier(tier);
        egui::Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), 40)
    }
}

/// Tier filter mode for the sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TierFilter {
    /// Show all traces.
    All,
    /// Show only traces with zero interesting decisions and status "ok".
    Clean,
    /// Show only traces with at least one interesting decision.
    HasIssues,
    /// Show only traces with status "error".
    Errors,
}

/// Main application state.
pub struct TraceViewerApp {
    /// The trace store holding all loaded traces.
    store: TraceStore,
    /// Currently selected trace ID.
    selected_trace: Option<String>,
    /// Currently expanded spans (by span_id).
    expanded_spans: BTreeMap<u64, bool>,
    /// Channel receiving file-watcher notifications.
    watcher_rx: mpsc::Receiver<()>,
    /// Kept alive to maintain the file watcher.
    _watcher: RecommendedWatcher,
    /// Last reload timestamp for UI display.
    last_reload: Instant,
    /// Number of traces loaded.
    trace_count: usize,
    /// Search filter for trace list.
    search_filter: String,
    /// Tier filter mode.
    tier_filter: TierFilter,
}

impl TraceViewerApp {
    /// Create the viewer app and start file watching.
    pub fn new(_cc: &eframe::CreationContext<'_>, trace_dir: PathBuf) -> Self {
        let mut store = TraceStore::new(trace_dir.clone());
        let trace_count = store.reload();

        let (tx, rx) = mpsc::channel();

        let watcher_tx = tx.clone();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let dominated_by_json = event.paths.iter().any(|p| {
                    p.extension().map(|e| e == "json").unwrap_or(false)
                });
                let is_relevant = matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                );
                if dominated_by_json && is_relevant {
                    let _ = watcher_tx.send(());
                }
            }
        })
        .expect("Failed to create file watcher");

        if trace_dir.exists() {
            watcher
                .watch(&trace_dir, RecursiveMode::NonRecursive)
                .expect("Failed to watch trace directory");
        } else {
            std::fs::create_dir_all(&trace_dir).ok();
            watcher
                .watch(&trace_dir, RecursiveMode::NonRecursive)
                .expect("Failed to watch trace directory");
        }

        Self {
            store,
            selected_trace: None,
            expanded_spans: BTreeMap::new(),
            watcher_rx: rx,
            _watcher: watcher,
            last_reload: Instant::now(),
            trace_count,
            search_filter: String::new(),
            tier_filter: TierFilter::All,
        }
    }

    /// Check for file watcher events and reload if needed.
    fn check_reload(&mut self) {
        let mut needs_reload = false;
        while self.watcher_rx.try_recv().is_ok() {
            needs_reload = true;
        }
        if needs_reload {
            self.trace_count = self.store.reload();
            self.last_reload = Instant::now();
        }
    }

    /// Render the left sidebar with trace list.
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading("Traces");
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
        });
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tier_filter, TierFilter::All, "All");
            ui.selectable_value(&mut self.tier_filter, TierFilter::Clean, "✅ Clean");
            ui.selectable_value(&mut self.tier_filter, TierFilter::HasIssues, "⚠ Issues");
            ui.selectable_value(&mut self.tier_filter, TierFilter::Errors, "❌ Errors");
        });
        ui.add_space(4.0);

        let all_traces = self.store.list_traces();
        let filtered: Vec<&TraceMeta> = all_traces
            .into_iter()
            .filter(|t| {
                let name_match = self.search_filter.is_empty()
                    || t.name.to_lowercase().contains(&self.search_filter.to_lowercase());
                let tier_match = match self.tier_filter {
                    TierFilter::All => true,
                    TierFilter::Clean => t.interesting_count == 0 && t.status == "ok",
                    TierFilter::HasIssues => t.interesting_count > 0,
                    TierFilter::Errors => t.status == "error",
                };
                name_match && tier_match
            })
            .collect();

        let status_text = format!("{}/{} traces · {:.0}s ago",
            filtered.len(),
            self.trace_count,
            self.last_reload.elapsed().as_secs_f64()
        );
        ui.label(egui::RichText::new(status_text).weak().small());
        ui.add_space(4.0);

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for trace in &filtered {
                let is_selected = self.selected_trace.as_deref() == Some(&trace.id);

                let tier_label = if trace.status == "error" {
                    "❌"
                } else if trace.interesting_count > 0 {
                    "⚠"
                } else {
                    "✅"
                };
                let label = format!("{} {} ({} dec, {} spans)",
                    tier_label,
                    trace.name,
                    trace.total_decisions,
                    trace.span_count,
                );

                let response = ui.selectable_label(is_selected, &label);
                if response.clicked() {
                    self.selected_trace = Some(trace.id.clone());
                    self.expanded_spans.clear();
                }
            }
        });
    }

    /// Render the main detail panel for the selected trace.
    fn render_detail(&mut self, ui: &mut egui::Ui) {
        let trace_id = match &self.selected_trace {
            Some(id) => id.clone(),
            None => {
                ui.centered_and_justified(|ui| {
                    ui.label(egui::RichText::new("Select a trace from the sidebar")
                        .weak()
                        .size(18.0));
                });
                return;
            }
        };

        let overview = match self.store.get_trace_overview(&trace_id) {
            Some(o) => o,
            None => {
                ui.label("Trace not found");
                return;
            }
        };

        ui.heading(&overview.meta.name);
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("Hash: 0x{:016X}", overview.meta.state_hash))
                .monospace()
                .weak());
            ui.separator();
            ui.label(format!("{} decisions", overview.meta.total_decisions));
            ui.separator();
            ui.label(format!("{} spans", overview.meta.span_count));
            if overview.meta.interesting_count > 0 {
                ui.separator();
                ui.label(egui::RichText::new(format!("⚠ {} interesting", overview.meta.interesting_count))
                    .color(TierColors::from_tier("Escalated")));
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.render_spans(ui, &trace_id, &overview.spans);
        });
    }

    /// Render the span tree with expandable decisions.
    fn render_spans(&mut self, ui: &mut egui::Ui, trace_id: &str, spans: &[SpanView]) {
        if spans.is_empty() {
            ui.label(egui::RichText::new("No spans recorded").weak());

            let decisions = self.store.get_span_decisions(trace_id, 0);
            if let Some(orphan_decisions) = decisions {
                if !orphan_decisions.is_empty() {
                    ui.add_space(4.0);
                    ui.label("Unspanned decisions:");
                    for d in &orphan_decisions {
                        self.render_decision(ui, d);
                    }
                }
            }
            return;
        }

        for span in spans {
            let is_expanded = self.expanded_spans.get(&span.span_id).copied().unwrap_or(false);

            let tier_color = TierColors::from_tier(&span.max_tier);
            let badge_bg = TierColors::badge_bg(&span.max_tier);

            let header = egui::RichText::new(format!(
                "{} {} — {} dec · {:.1}ms",
                if is_expanded { "▼" } else { "▶" },
                span.name,
                span.total_decisions,
                span.duration_micros as f64 / 1000.0,
            ));

            let response = ui.horizontal(|ui| {
                let resp = ui.label(header);

                let badge_rect = egui::Rect::from_min_size(
                    resp.rect.right_top() + egui::vec2(8.0, 2.0),
                    egui::vec2(80.0, 16.0),
                );
                ui.painter().rect_filled(badge_rect, 4.0, badge_bg);
                ui.painter().text(
                    badge_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &span.max_tier,
                    egui::FontId::proportional(11.0),
                    tier_color,
                );

                resp
            });

            if header_clicked(ui, &response.inner) {
                self.expanded_spans.insert(span.span_id, !is_expanded);
            }

            if is_expanded {
                ui.indent(format!("span_{}", span.span_id), |ui| {
                    let decisions = self.store.get_span_decisions(trace_id, span.span_id);
                    match decisions {
                        Some(decs) if !decs.is_empty() => {
                            for d in &decs {
                                self.render_decision(ui, d);
                            }
                        }
                        _ => {
                            ui.label(egui::RichText::new("No decisions in this span").weak());
                        }
                    }
                });
            }

            ui.add_space(2.0);
        }
    }

    /// Render a single decision row.
    fn render_decision(&self, ui: &mut egui::Ui, d: &DecisionView) {
        let tier_color = TierColors::from_tier(&d.tier);

        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("#{}", d.id))
                .monospace()
                .weak()
                .small());

            ui.label(egui::RichText::new(&d.kind)
                .color(tier_color)
                .strong());

            ui.label(egui::RichText::new(format!("m={:.4}", d.margin))
                .monospace()
                .weak());

            if !d.entity.is_empty() && d.entity != "None" {
                ui.label(egui::RichText::new(&d.entity)
                    .weak()
                    .small());
            }
        });
    }
}

/// Check if a label response was clicked (for the span header).
fn header_clicked(_ui: &egui::Ui, response: &egui::Response) -> bool {
    response.clicked()
}

impl eframe::App for TraceViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_reload();

        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        configure_visuals(ctx);

        egui::SidePanel::left("trace_list")
            .default_width(280.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_detail(ui);
        });
    }
}

/// Configure dark theme with custom colors.
fn configure_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(24, 24, 28);
    visuals.window_fill = egui::Color32::from_rgb(30, 30, 34);
    visuals.extreme_bg_color = egui::Color32::from_rgb(18, 18, 22);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(36, 36, 42);
    ctx.set_visuals(visuals);
}
