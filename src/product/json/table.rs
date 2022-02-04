use sea_query::Iden;

#[derive(Iden)]
pub enum Products {
    Table,
    Id,
    Name,
    Currency,
    Price,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
