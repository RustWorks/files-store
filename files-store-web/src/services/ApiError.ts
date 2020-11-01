export interface ApiError {
  code: string
  message: string
  params: Record<string, string | number>
}

export interface ApiErrors {
  message: string
  errors?: Record<string, ApiError[]>
}
