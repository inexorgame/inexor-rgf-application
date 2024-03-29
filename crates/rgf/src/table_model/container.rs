use std::marker::PhantomData;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

pub(crate) trait TableContainer {
    fn table(&self) -> Table;
}

pub(crate) trait TableOptions: Sized {
    fn options(table: &mut Table) -> &mut Table;
}

pub(crate) struct DefaultTableOptions {}

impl TableOptions for DefaultTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}

pub(crate) struct DefaultTableContainer<S, T: Clone + Tabled + From<S>, O: TableOptions>(pub(crate) Vec<T>, PhantomData<S>, PhantomData<O>);

impl<S: 'static, T: Clone + Tabled + From<S> + 'static, O: TableOptions + 'static> DefaultTableContainer<S, T, O> {
    pub(crate) fn into_boxed(self) -> Box<DefaultTableContainer<S, T, O>> {
        Box::new(self)
    }
}

impl<S, T: Clone + Tabled + From<S>, O: TableOptions> TableContainer for DefaultTableContainer<S, T, O> {
    fn table(&self) -> Table {
        let mut table = Table::new(self.0.to_vec().iter());
        O::options(&mut table).to_owned()
    }
}

impl<S, T: Clone + Tabled + From<S>, O: TableOptions> From<Vec<S>> for DefaultTableContainer<S, T, O> {
    fn from(entries: Vec<S>) -> Self {
        DefaultTableContainer::<S, T, O>(entries.into_iter().map(From::from).collect(), PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S>, O: TableOptions> From<S> for DefaultTableContainer<S, T, O> {
    fn from(s: S) -> Self {
        DefaultTableContainer::<S, T, O>(vec![s.into()], PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S>, O: TableOptions> ToString for DefaultTableContainer<S, T, O> {
    fn to_string(&self) -> String {
        self.table().to_string()
    }
}
