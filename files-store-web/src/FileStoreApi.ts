import type { FsNode } from "./FsNode"

export function getFiles(uuid?: string): Promise<FsNode[]> {
  const path = uuid ? `api/files/${uuid}` : "api/files"
  return fetch(`http://localhost:2000/${path}`).then(response => response.json())
}
