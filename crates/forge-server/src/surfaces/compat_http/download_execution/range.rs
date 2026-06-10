use crate::{
    ForgeServerCompatibilityPreparedRequest, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerRangeRequest {
    Full,
    Bytes { start: u64, end_inclusive: u64 },
    Suffix { len: u64 },
    OpenEnded { start: u64 },
}

impl ForgeServerRangeRequest {
    pub(crate) fn from_prepared_request(
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
    ) -> Result<Self, ForgeServerQueryHandoffDenial> {
        let values = prepared_request
            .request_contract()
            .canonical_headers()
            .values("range");
        match values {
            None => Ok(Self::Full),
            Some([]) => Ok(Self::Full),
            Some([value]) => Self::parse(prepared_request, value),
            Some(_) => Err(download_request_invalid(
                prepared_request,
                "binary download accepts at most one Range header value",
            )),
        }
    }

    pub fn canonical_digest(&self) -> String {
        match self {
            Self::Full => "compat-http-range-v1|full".to_string(),
            Self::Bytes {
                start,
                end_inclusive,
            } => format!("compat-http-range-v1|bytes={start}-{end_inclusive}"),
            Self::Suffix { len } => format!("compat-http-range-v1|suffix={len}"),
            Self::OpenEnded { start } => format!("compat-http-range-v1|open={start}-"),
        }
    }

    pub(crate) fn resolve(
        &self,
        total_bytes: usize,
        range_admitted: bool,
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
    ) -> Result<(usize, usize, bool), ForgeServerQueryHandoffDenial> {
        if !range_admitted {
            return Ok((0, total_bytes, false));
        }
        match self {
            Self::Full => Ok((0, total_bytes, false)),
            Self::Bytes {
                start,
                end_inclusive,
            } => {
                let start = *start as usize;
                let end_inclusive = *end_inclusive as usize;
                if start >= total_bytes {
                    return Err(download_request_invalid(
                        prepared_request,
                        "range start exceeds the available binary representation length",
                    ));
                }
                if end_inclusive >= total_bytes {
                    return Err(download_request_invalid(
                        prepared_request,
                        "range end exceeds the available binary representation length",
                    ));
                }
                if end_inclusive < start {
                    return Err(download_request_invalid(
                        prepared_request,
                        "range end must not sort before range start",
                    ));
                }
                Ok((start, end_inclusive + 1, true))
            }
            Self::Suffix { len } => {
                if *len == 0 {
                    return Err(download_request_invalid(
                        prepared_request,
                        "suffix ranges must request at least one byte",
                    ));
                }
                let len = (*len as usize).min(total_bytes);
                Ok((
                    total_bytes.saturating_sub(len),
                    total_bytes,
                    len < total_bytes,
                ))
            }
            Self::OpenEnded { start } => {
                let start = *start as usize;
                if start >= total_bytes {
                    return Err(download_request_invalid(
                        prepared_request,
                        "open-ended range start exceeds the available binary representation length",
                    ));
                }
                Ok((start, total_bytes, start > 0))
            }
        }
    }

    fn parse(
        prepared_request: &ForgeServerCompatibilityPreparedRequest,
        value: &str,
    ) -> Result<Self, ForgeServerQueryHandoffDenial> {
        let value = value.trim();
        let Some(spec) = value.strip_prefix("bytes=") else {
            return Err(download_request_invalid(
                prepared_request,
                "binary download only admits bytes= shaped range units",
            ));
        };
        if spec.contains(',') {
            return Err(download_request_invalid(
                prepared_request,
                "multi-range requests are not admitted on the canonical binary egress lane",
            ));
        }
        let Some((start_raw, end_raw)) = spec.split_once('-') else {
            return Err(download_request_invalid(
                prepared_request,
                "range request must include exactly one dash separator",
            ));
        };
        if start_raw.is_empty() {
            let len = parse_u64(prepared_request, end_raw, "suffix range length")?;
            return Ok(Self::Suffix { len });
        }
        let start = parse_u64(prepared_request, start_raw, "range start")?;
        if end_raw.is_empty() {
            return Ok(Self::OpenEnded { start });
        }
        let end_inclusive = parse_u64(prepared_request, end_raw, "range end")?;
        Ok(Self::Bytes {
            start,
            end_inclusive,
        })
    }
}

fn parse_u64(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    raw: &str,
    label: &str,
) -> Result<u64, ForgeServerQueryHandoffDenial> {
    raw.trim().parse::<u64>().map_err(|_| {
        download_request_invalid(
            prepared_request,
            format!("binary download {label} must be an unsigned integer"),
        )
    })
}

fn download_request_invalid(
    prepared_request: &ForgeServerCompatibilityPreparedRequest,
    detail: impl Into<String>,
) -> ForgeServerQueryHandoffDenial {
    ForgeServerQueryHandoffDenial::new(
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        prepared_request
            .admission()
            .request_context()
            .diagnostics_profile(),
        detail,
    )
}
