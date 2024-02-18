use handlebars::{Context as HbsContext, Handlebars};

use anyhow::{Context, Result};

use super::config::BinserveConfig;

/// Prepare the partials and template variables for handlebars at initialization.
pub fn render_templates(config: &BinserveConfig) -> Result<(Handlebars<'static>, HbsContext)> {
    let mut handlebars_reg = Handlebars::new();

    // register the context with the template variables
    let hbs_context = HbsContext::wraps(&config.template.variables)?;

    // prepare template partials
    for (partial_name, template_path) in &config.template.partials {
        // register the partial templates
        let partial_template = std::fs::read_to_string(template_path).with_context(|| {
            format!(
                "Failed to read Handlebars partial file: {:?}",
                template_path
            )
        })?;

        handlebars_reg.register_partial(partial_name, partial_template)?;
    }

    Ok((handlebars_reg, hbs_context))
}
