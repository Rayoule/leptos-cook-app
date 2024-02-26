use leptos::*;

pub struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[component]
fn RecipeSheet(
    recipe: Recipe,
) -> impl IntoView {

    let ingredient_list = (0..recipe.ingredients.len())
        .map(move |i| view! { <li>{recipe.ingredients[i].clone()}</li> })
        .collect_view();
    let instructions_list = (0..recipe.instructions.len())
        .map(move |i| view! { <li>{recipe.instructions[i].clone()}</li> })
        .collect_view();

    view! {
        <div>
            <h1>{recipe.name}</h1>
            <h2>Ingredients</h2>
            <ul>{ingredient_list}</ul>
            <h2>Instructions</h2>
            <ul>{instructions_list}</ul>
        </div>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;

    let super_list = vec![0,1,2];

    let rec = Recipe {
        name: "Galettes de Sarrasin".to_owned(),
        ingredients: vec!["Farine de Sarrasin".to_owned(), "Sel".to_owned(), "Eau".to_owned(), "Beurre".to_owned()],
        instructions: vec!["Melanger le tout".to_owned(), "Attendre".to_owned(), "Poeller avec du beurre".to_owned()],
    };

    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
                set_count.update(|n| *n += 1);
            }
            //class:red=move || count.get() % 2 == 1
            style="position: absolute"
            style:left=move || format!("{}px", x.get() + 100)
            style:background-color=move || format!("rgb({}, 100, 100)", x.get())
            style:max-width="400px"
            style=("--columns", x)
        >
            "Click me: "
            {move || count.get()}
        </button>

        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
        <p>
            "Double count: "
            {double_count}
        </p>

        <p>{super_list}</p>

        <RecipeSheet recipe=rec/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
