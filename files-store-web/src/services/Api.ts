import { FsNodesService } from "./FsNodesService"
import { navigate } from "@bjornlu/svelte-router"

import type { ApiStorage } from "./ApiStorage"
import type { ServiceConfig } from "./ApiService"
import { AuthService } from "./AuthService"

const storage: ApiStorage = {
  getItemSync: key => localStorage.getItem(key),
  getItem: key => Promise.resolve(localStorage.getItem(key)),
  setItem: (key, value) => Promise.resolve(localStorage.setItem(key, value)),
  removeItem: key => Promise.resolve(localStorage.removeItem(key)),
  clear: () => Promise.resolve(localStorage.clear())
}

function onUnauthorized() {
  storage
    .clear()
    .then(() => {})
    .catch(() => {})
  navigate("#/login")
}

const config: ServiceConfig = { onUnauthorized, storage, baseURL: "http://localhost:4200" }

export const Api = {
  auth: new AuthService(config),
  fsNodes: new FsNodesService(config)
}
