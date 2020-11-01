import Axios, { AxiosInstance, AxiosRequestConfig } from "axios"

import type { ApiStorage } from "./ApiStorage"

export interface ServiceConfig {
  baseURL?: string
  storage: ApiStorage
  onUnauthorized: () => void
}

export class ApiService {
  protected readonly STORAGE_AUTH_TOKEN_KEY = "AUTH_TOKEN"
  protected readonly instance: AxiosInstance
  protected readonly storage: ApiStorage
  protected readonly config: ServiceConfig
  protected readonly onUnauthorized: () => void = () => {}

  public constructor(config: ServiceConfig) {
    const { baseURL, storage, onUnauthorized } = config
    this.storage = storage
    this.onUnauthorized = onUnauthorized
    this.config = config
    this.instance = Axios.create({
      baseURL,
      timeout: 5 * 60 * 1000,
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json"
      }
    })

    this.instance.interceptors.request.use(
      config =>
        this.storage.getItem(this.STORAGE_AUTH_TOKEN_KEY).then(token => {
          if (token) {
            // tslint:disable-next-line: no-unsafe-any
            config.headers.Authorization = `Bearer ${token}`
          }
          return config
        }),
      error => Promise.reject(error)
    )

    this.instance.interceptors.response.use(
      response => {
        if (response.status === 401) {
          this.onUnauthorized()
        }
        return response
      },
      (error: Error) => {
        if (error.message === "Request failed with status code 401") {
          this.onUnauthorized()
        }
        return Promise.reject(error)
      }
    )
  }

  protected request<T>(config: AxiosRequestConfig): Promise<T> {
    return this.instance
      .request<T>({ ...config })
      .then(response => response.data)
      .catch(error =>
        // tslint:disable-next-line: no-unsafe-any
        Promise.reject(error.response.data)
      )
  }
}
