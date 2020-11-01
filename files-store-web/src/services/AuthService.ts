import type { AxiosRequestConfig, CancelTokenSource } from "axios"

import { ApiService } from "./ApiService"
import type { Login } from "../models/Login"
import type { PasswordAuthResponse } from "../models/PasswordAuthResponse"
import type { PasswordCredentialsPayload } from "../models/PasswordCredentialsPayload"
import type { Signup } from "../models/Signup"
import type { User } from "../models/User"
import type { UserUpdatePayload } from "../models/UserUpdatePayload"
import type { ChangePasswordPayload } from "../models/ChangePasswordPayload"

export class AuthService extends ApiService {
  public login(login: Login): Promise<PasswordAuthResponse> {
    const passwordCredentialsPayload: PasswordCredentialsPayload = {
      ...login,
      grant_type: "password"
    }
    return this.authenticate({ url: "/api/users/login", method: "POST", data: passwordCredentialsPayload })
  }

  public signup(signup: Signup): Promise<PasswordAuthResponse> {
    return this.authenticate({ url: "/api/users/signup", method: "POST", data: signup })
  }

  public logout(): Promise<void> {
    return this.storage
      .removeItem(this.STORAGE_AUTH_TOKEN_KEY)
      .then(() => this.onUnauthorized())
      .catch(() => this.onUnauthorized())
  }

  public getMe(cancelToken?: CancelTokenSource): Promise<User> {
    return this.request({
      url: "/api/users/me",
      method: "GET",
      cancelToken: cancelToken?.token
    })
  }

  public updateUser(userUpdatePayload: UserUpdatePayload): Promise<User> {
    return this.request({
      url: "/api/users",
      method: "PUT",
      data: userUpdatePayload
    })
  }

  public changePassword(changePasswordPayload: ChangePasswordPayload): Promise<User> {
    return this.request({
      url: "/api/users/password",
      method: "PUT",
      data: changePasswordPayload
    })
  }

  private authenticate(config: AxiosRequestConfig): Promise<PasswordAuthResponse> {
    return this.instance
      .request<PasswordAuthResponse>(config)
      .then(response =>
        this.storage.setItem(this.STORAGE_AUTH_TOKEN_KEY, response.data.access_token).then(() => response.data)
      )
      .catch(error =>
        // tslint:disable-next-line: no-unsafe-any
        Promise.reject(error.response.data)
      )
  }
}
