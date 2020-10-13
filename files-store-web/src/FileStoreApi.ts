import type { FsNode } from "./FsNode"

export function getFiles(token: string): Promise<FsNode[]> {
  return fetch("http://localhost:2000/api/files", { headers: {
    token
  } }).then(response => response.json())
}
