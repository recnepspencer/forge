macro_rules! define_materialization_admission_outcome {
    ($case:ident, $outcome:ident, $view:ident, $case_id:ident, $cases:ident, [$($denial:expr),* $(,)?]) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $case {
            Admitted(AdmittedLayoutMaterialization),
            Denied(MaterializationDenial),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub struct $outcome {
            case: $case,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $case_id {
            Admitted,
            Denied($crate::materialization::MaterializationDenialKind),
        }

        impl $case_id {
            pub const fn as_str(self) -> &'static str {
                match self {
                    Self::Admitted => "admitted",
                    Self::Denied(denial) => denial.as_str(),
                }
            }
        }

        pub fn $cases() -> impl Iterator<Item = $case_id> {
            std::iter::once($case_id::Admitted)
                .chain([$($case_id::Denied($denial)),*].into_iter())
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $view<'a> {
            Admitted(&'a AdmittedLayoutMaterialization),
            Denied(&'a MaterializationDenial),
        }

        impl $outcome {
            fn issue(result: Result<AdmittedLayoutMaterialization, MaterializationDenial>) -> Self {
                let case = match result {
                    Ok(materialization) => $case::Admitted(materialization),
                    Err(denial) => $case::Denied(denial),
                };
                Self { case }
            }

            pub const fn view(&self) -> $view<'_> {
                match &self.case {
                    $case::Admitted(materialization) => $view::Admitted(materialization),
                    $case::Denied(denial) => $view::Denied(denial),
                }
            }

            pub const fn case_id(&self) -> $case_id {
                match &self.case {
                    $case::Admitted(_) => $case_id::Admitted,
                    $case::Denied(denial) => $case_id::Denied(denial.kind()),
                }
            }

            pub fn into_result(
                self,
            ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
                match self.case {
                    $case::Admitted(materialization) => Ok(materialization),
                    $case::Denied(denial) => Err(denial),
                }
            }

            pub fn unwrap(self) -> AdmittedLayoutMaterialization {
                self.into_result().unwrap()
            }

            pub fn expect(self, message: &str) -> AdmittedLayoutMaterialization {
                self.into_result().expect(message)
            }

            pub fn unwrap_err(self) -> MaterializationDenial {
                self.into_result().unwrap_err()
            }
        }

        impl PartialEq<Result<AdmittedLayoutMaterialization, MaterializationDenial>> for $outcome {
            fn eq(
                &self,
                other: &Result<AdmittedLayoutMaterialization, MaterializationDenial>,
            ) -> bool {
                match (self.view(), other) {
                    ($view::Admitted(left), Ok(right)) => left == right,
                    ($view::Denied(left), Err(right)) => left == right,
                    _ => false,
                }
            }
        }
    };
}
