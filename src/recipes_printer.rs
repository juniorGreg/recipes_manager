use tectonic;
use crate::models::*;

async fn generate_latex() -> String {
    let latex = r#"
\documentclass[a4paper,17pt]{extarticle}
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

\section*{Panini Chèvre, Légumes grillés et Pesto}
\subsection*{Ingredients :}
\begin{itemize}[parsep=1pt]
    \item Pains Ciabatta 3 X 6
    \item 1 kg Fromage de chèvre
    \item 500 gr Pesto de basilic
    \item 500 gr Poivrons rôtis 
    \item 500 gr Courgettes rôties
    \item 200 gr Tomates séchées
    \item 200 gr Oignons caramélisés
    \item 20 gr Ail hachés
    \item Sel/Poivron (au goût)
\end{itemize}
\subsection*{Marche à suivre :}
\begin{enumerate}[parsep=1pt]
    \item Caraméliser les oignons avec de l'huile d'olives ou du beurre.
    \item Peser les ingrédients.
    \item Couper en morceaux les poivrons et les courgettes.
    \item Mélanger tous les ingrédients.
    \item Répartir le mélange sur environ 25 pains.
\end{enumerate}
\end{document}
    "#;
    String::from(latex) 
}

pub async fn print_pdf_recipe(_recipe: &RecipeWithId) -> Vec<u8> {
    let latex = generate_latex().await;
    let pdf_bytes = tectonic::latex_to_pdf(latex).unwrap();
    dbg!("okidoo2");
    pdf_bytes 
}
