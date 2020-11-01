export interface ApiStorage {
  getItem: (key: string) => Promise<string | null>
  getItemSync: (key: string) => string
  setItem: (key: string, value: string) => Promise<void>
  removeItem: (key: string) => Promise<void>
  clear: () => Promise<void>
}
