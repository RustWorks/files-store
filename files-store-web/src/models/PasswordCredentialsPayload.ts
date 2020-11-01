export interface PasswordCredentialsPayload {
  grant_type: "password"
  username: string
  password: string
  scope?: string
}
