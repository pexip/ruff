pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use anyhow::Result;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::settings::LinterSettings;
    use crate::test::test_path;
    use crate::assert_messages;

    #[test_case(Rule::NOQAByCode, Path::new("PEX102.py"))]
    #[test_case(Rule::NOQAByName, Path::new("PEX103.py"))]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("pexip").join(path).as_path(),
            &LinterSettings::for_rule(rule_code),
        )?;
        assert_messages!(snapshot, diagnostics);
        Ok(())
    }

    #[test]
    fn noqa_names() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pexip/noqa_names.py"),
            &LinterSettings::for_rules(vec![
                Rule::UnusedVariable,
                Rule::AmbiguousVariableName,
            ]),
        )?;
        assert_messages!(diagnostics);
        Ok(())
    }

    #[test]
    fn noqa_names_simple() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pexip/noqa_names_simple.py"),
            &LinterSettings::for_rules(vec![
                Rule::UnusedVariable,
                Rule::AmbiguousVariableName,
            ]),
        )?;
        assert_messages!(diagnostics);
        Ok(())
    }

    #[test]
    fn noqa_mixed() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pexip/noqa_mixed.py"),
            &LinterSettings::for_rules(vec![
                Rule::UnusedVariable,
                Rule::AmbiguousVariableName,
            ]),
        )?;
        assert_messages!(diagnostics);
        Ok(())
    }

    #[test]
    fn ruff_noqa_names() -> Result<()> {
        let diagnostics = test_path(
            Path::new("pexip/ruff_noqa_names.py"),
            &LinterSettings::for_rules(vec![
                Rule::UnusedVariable,
                Rule::AmbiguousVariableName,
            ]),
        )?;
        assert_messages!(diagnostics);
        Ok(())
    }
}
