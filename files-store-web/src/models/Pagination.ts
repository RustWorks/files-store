export interface Pagination {
  limit: number
  offset: number
}

export const defaultPagination: Pagination = {
  limit: 10,
  offset: 0
}
