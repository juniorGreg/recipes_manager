use tectonic;
use crate::models::*;
use crate::ingredient_list_manager::IngredientList;

async fn header_latex(pt: Option<String>) -> String {
    let pt = pt.unwrap_or("17".to_owned());
    let mut latex: String = r#"
\documentclass[a4paper,"#.to_owned();

    latex.push_str(&pt);
    latex.push_str(r#"pt]{extarticle}
\usepackage[T1]{fontenc}
\usepackage{graphicx}
\usepackage{fontspec}
\usepackage[margin=2cm]{geometry}
\usepackage{enumitem}

\setmainfont{Arial}
\pagenumbering{gobble}

\begin{document}

\begin{center}
    \includegraphics[height=3cm]{template_pdf/banniere_benelux}
\end{center}

\section*{"#);

    String::from(latex)
}


async fn generate_latex(recipe: &RecipeWithId) -> String {
    let mut latex = header_latex(None).await;
    latex.push_str(&recipe.title);
    latex.push_str(r#"}
\subsection*{Ingredients :}
\begin{itemize}[parsep=1pt]
"#);
    for ingredient in &recipe.ingredients {
        let ingredient_str = format!("\\item {} {} {}", ingredient.ingredient_quantity,
                                                    ingredient.ingredient_unit,
                                                    ingredient.ingredient_name);
        latex.push_str(&ingredient_str);
    }

    latex.push_str(r#"\end{itemize}
\subsection*{Marche à suivre :}
\begin{enumerate}[parsep=1pt]"#);

    for prep in &recipe.preparation_steps {
        let prep_str = format!("\\item {}", prep);
        latex.push_str(&prep_str);
    }
    latex.push_str(r#"
\end{enumerate}"#);
    latex.push_str(r#"
\end{document}"#);
    String::from(latex) 
}

async fn generate_latex_for_ingredient_list(ingredients_lists: &Vec<IngredientList>) -> String {
    let mut latex = header_latex(Some("15".to_owned())).await;
    latex.push_str("Listes des ingrédients");
    latex.push_str(r#"}"#);
    
    for ingredients_list in ingredients_lists {
        latex.push_str(&format!(r#"\subsection*{{{}}}"#, ingredients_list.recipe_name));
        latex.push_str(r#"\subsubsection*{Ingrédients}"#);
        
        let mut ingredients_str = ingredients_list.ingredients.join(", ");
            ingredients_str.push_str(".");
            latex.push_str(&ingredients_str);

            if ingredients_list.allergens.len() > 0 {
            latex.push_str(r#"\subsubsection*{Allergène}"#);

            let mut allergens_str = ingredients_list.allergens.join(", ");
            allergens_str.push_str(".");

            latex.push_str(&allergens_str);
        }

        latex.push_str(r#"\bigskip"#);
    }

    latex.push_str(r#"
\end{document}"#);
    String::from(latex) 
}

pub async fn print_pdf_recipe(recipe: &RecipeWithId) -> Vec<u8> {
    let latex = generate_latex(recipe).await;
    let pdf_bytes = tectonic::latex_to_pdf(latex).unwrap();
    dbg!("okidoo2");
    pdf_bytes 
}

pub async fn print_ingredients_lists(ingredients_lists: &Vec<IngredientList>) -> Vec<u8> {
    let latex = generate_latex_for_ingredient_list(ingredients_lists).await;
    let pdf_bytes = tectonic::latex_to_pdf(latex).unwrap();
    pdf_bytes
}
