import type { ApiError, ApiErrors } from "./services/ApiError"

export function getFieldErrorMessage(
  field: string,
  intl: (key: string, params?: Record<string, string | number>) => string,
  errors?: ApiErrors
): string | undefined {
  if (errors) {
    const fieldsErrors = errors.errors && (errors.errors[field] as ApiError[] | undefined)
    if (fieldsErrors && fieldsErrors.length > 0) {
      const error = fieldsErrors[0]

      return intl(error.message, error.params)
    } else {
      return undefined
    }
  } else {
    return undefined
  }
}
