use std::collections::HashMap;

use crate::components::{no_content::NoContent, table::pagination::Pagination};
use yew::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct TableProps {
    pub columns: Vec<Column>,
    pub data: Vec<HashMap<String, TableCellData>>,
    pub page_size: usize,
    pub on_row_click: Option<Callback<HashMap<String, TableCellData>>>,
    pub on_row_delete: Option<Callback<HashMap<String, TableCellData>>>,
    pub on_row_edit: Option<Callback<HashMap<String, TableCellData>>>,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub deletable: bool,
}

impl TableProps {
    /// Paginate the provided data and return the current page, total pages and the current data, we can give it sorted data or unsorted data
    pub fn paginate(
        &self,
        current_page: usize,
        data: Vec<HashMap<String, TableCellData>>,
    ) -> (usize, usize, Vec<HashMap<String, TableCellData>>) {
        // Get the total number of pages, rounding up to the nearest whole number e.g 5/2 = 2.5 => 3
        let total_pages = (self.data.len() as f64 / self.page_size as f64).ceil() as usize;

        let current_page = current_page;
        let current_data = data
            .iter()
            .skip((current_page - 1) * self.page_size)
            .take(self.page_size)
            .cloned()
            .collect();
        (current_page, total_pages, current_data)
    }

    /// Sort the immutable data and return a new sorted vector, this is the main source of truth, we want to keep the original data immutable
    pub fn sort(&self, column: &Column) -> Vec<HashMap<String, TableCellData>> {
        let mut data = self.data.clone();
        match column.sort_order {
            SortOrder::Ascending => {
                data.sort_by(|a, b| match (a.get(&column.name), b.get(&column.name)) {
                    (Some(TableCellData::String(a)), Some(TableCellData::String(b))) => a.cmp(b),
                    (Some(TableCellData::Integer(a)), Some(TableCellData::Integer(b))) => a.cmp(b),
                    (Some(TableCellData::Float(a)), Some(TableCellData::Float(b))) => a.partial_cmp(b).unwrap(),
                    _ => std::cmp::Ordering::Equal,
                });
            }
            SortOrder::Descending => {
                data.sort_by(|a, b| match (a.get(&column.name), b.get(&column.name)) {
                    (Some(TableCellData::String(a)), Some(TableCellData::String(b))) => b.cmp(a),
                    (Some(TableCellData::Integer(a)), Some(TableCellData::Integer(b))) => b.cmp(a),
                    (Some(TableCellData::Float(a)), Some(TableCellData::Float(b))) => b.partial_cmp(a).unwrap(),
                    _ => std::cmp::Ordering::Equal,
                });
            }
            SortOrder::Default => {
                // Do nothing
            }
        }
        data
    }
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Column {
    pub name: String,
    #[prop_or_default]
    pub sortable: bool,
    #[prop_or_default]
    pub sort_order: SortOrder,
    // #[prop_or_default]
    pub sort_icon: IconId,
}

#[allow(dead_code, unused)]
#[derive(PartialEq, Clone, Debug)]
pub enum TableCellData {
    String(String),
    Integer(i64),
    Html(Html),
    Float(f64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum SortOrder {
    Default,
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Default
    }
}

impl Column {
    pub fn toggle_sort(&mut self) {
        self.sort_order = match self.sort_order {
            SortOrder::Default => SortOrder::Ascending,
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Default,
        }
    }

    pub fn toggle_sort_icon(&mut self) {
        log::info!("{:?}", self.sort_order);
        self.sort_icon = match self.sort_order {
            SortOrder::Default => IconId::BootstrapFilter,
            SortOrder::Ascending => IconId::BootstrapSortUp,
            SortOrder::Descending => IconId::BootstrapSortDown,
        }
    }
}

#[function_component]
pub fn DataTable(props: &TableProps) -> Html {
    let pagination_state = use_state_eq(|| props.paginate(1, props.data.clone()));
    let working_data = use_state_eq(|| props.data.clone());
    let working_columns = use_state_eq(|| props.columns.clone());

    let props_clone = props.clone();

    let on_page_change = {
        let pagination_state = pagination_state.clone();
        let working_data = working_data.clone();
        Callback::from(move |page: usize| {
            pagination_state.set(props_clone.paginate(page, working_data.clone().to_vec()));
        })
    };

    // Sort the data when a column is clicked
    let on_click_sort = {
        let pagination_state = pagination_state.clone();
        let props = props.clone();
        let working_data = working_data.clone();
        let working_columns = working_columns.clone();
        // outer callback to capture the column reference
        Callback::from(move |column: &Column| {
            let pagination_state = pagination_state.clone();
            let props = props.clone();
            let column_clone = column.clone();
            let working_data = working_data.clone();
            let working_columns = working_columns.clone();
            // inner closure to capture the click(MouseEvent) event
            Callback::from(move |_| {
                let mut column_clone_mut = column_clone.clone();
                column_clone_mut.toggle_sort();
                column_clone_mut.toggle_sort_icon();
                let sorted_data = props.sort(&column_clone_mut);
                working_data.set(sorted_data.clone());
                let mut working_columns_vec = (*working_columns).clone();

                if let Some(c) = working_columns_vec.iter_mut().find(|c| c.name == column_clone_mut.name) {
                    *c = column_clone_mut.clone();
                }

                working_columns.set(working_columns_vec);

                pagination_state.set(props.paginate(1, sorted_data));
            })
        })
    };

    let on_click_edit = {
        let props = props.clone();
        // outer callback to capture the row data
        // the Callback::noop() is a placeholder for the on_row_edit prop so that it does not panic if it is not provided on initialization
        let on_click_edit = props.on_row_edit.clone().unwrap_or(Callback::noop());
        Callback::from(move |row_data: HashMap<String, TableCellData>| {
            let on_click_edit = on_click_edit.clone();

            Callback::from(move |_| {
                on_click_edit.emit(row_data.clone());
            })
        })
    };

    let on_row_click = {
        let props = props.clone();
        let on_click_row = props.on_row_click.clone().unwrap_or(Callback::noop());

        Callback::from(move |row_data: HashMap<String, TableCellData>| {
            let on_click_row = on_click_row.clone();

            Callback::from(move |_| {
                on_click_row.emit(row_data.clone());
            })
        })
    };

    // Listen for any changes to props and paginate again
    let props_for_deps = props.clone();
    let props_for_effect = props.clone();
    let pagination_state_clone = pagination_state.clone();
    let working_data_clone = working_data.clone();
    let working_columns_clone = working_columns.clone();
    use_effect_with_deps(move |_| {
        pagination_state_clone.set(props_for_effect.paginate(1, props_for_effect.data.clone()));
        working_data_clone.set(props_for_effect.data.clone());
        working_columns_clone.set(props_for_effect.columns);
        || ()
    }, props_for_deps);

    html! {
        <div class="w-full flex flex-col justify-between h-5/6">
            <table class="border-collapse border rounded table-fixed w-full h-full text-gray-500 mt-4 mb-4 min-w-52 text-md">
                <thead>
                    <tr class="p-2">
                    { working_columns.iter().map(|column| html! { if column.sortable { <th class="border-b p-2 border-gray-200 text-nowrap font-bold text-gray-800 text-left cursor-pointer" onclick={on_click_sort.emit(column)}><span class="flex flex-row items-center"><span>{ &column.name }</span><span class="text-blue-500"><Icon width={"0.8em".to_owned()} height={"0.8em".to_owned()} icon_id={column.sort_icon}/></span></span></th> } else { <th class="border-b p-2 border-gray-200 text-nowrap font-bold text-gray-800 text-left cursor-pointer"><span class="flex flex-row items-center"><span>{ &column.name }</span></span></th> } }).collect::<Html>() }
                    {
                        if props.editable || props.deletable {
                            html! {<th class="border-b p-2 border-gray-200 text-nowrap  font-bold text-gray-800 text-left">{"Actions"}</th>}
                        } else {
                            html! {}
                        }
                    }
                </tr>
                </thead>
                <tbody>
                    {
                        pagination_state.2.iter().map(|row_data| html! {
                            <tr onclick={on_row_click.emit(row_data.clone())} class="border-b border-gray-200 p-2">
                                // Assuming each T can be displayed, or you implement logic to extract the values based on column id or name.
                                // This part highly depends on the structure of T and how you plan to access its data.
                                {
                                    for props.columns.iter().map(|column| html! {
                                        <td class="p-2">{
                                            match row_data.get(&column.name) {
                                                Some(TableCellData::String(s)) => html! {{s}},
                                                Some(TableCellData::Integer(i)) => html! {{i}},
                                                Some(TableCellData::Html(j)) => {
                                                    // For actual HTML content rendering, you would typically use a Yew component or a function component.
                                                    j.to_owned()
                                                },
                                                Some(TableCellData::Float(f)) => html! {{f}},
                                                None => html! {"N/A"}, // Render a placeholder string for missing data
                                            }
                                        }</td>
                                    })
                                }
                                {
                                    if props.editable || props.deletable {
                                        html!{
                                            <td class="flex flex-row items-center gap-2 h-full py-2">
                                               if props.editable {
                                                   <button onclick={on_click_edit.emit(row_data.clone())} class="text-blue-500 cursor-pointer"><Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapPencil}/></button>
                                               } else {

                                               }

                                                  if props.deletable {
                                                    <button class="text-theme-red cursor-pointer"><Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapTrash}/></button>
                                                  } else {

                                                  }
                                            </td>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                            </tr>
                        }).collect::<Html>()
                    }
                    {
                        // No content to display
                        if pagination_state.2.is_empty() {
                            html! {<tr><td colspan={props.columns.len().to_string()} class="py-2"><NoContent /></td>{ if props.deletable || props.editable { html!{<td></td>} } else { html!{} } }</tr>}
                        } else {
                            html! {}
                        }
                    }
                </tbody>
            </table>
            <Pagination
                current_page={pagination_state.0}
                total_pages={pagination_state.1}
                on_page_change={on_page_change}
            />
        </div>
    }
}
