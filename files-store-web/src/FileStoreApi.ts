import type { FsNodesResponse, FsNode, UploadResult } from "./FsNode"

const baseUrl = "http://localhost:4200"

export function getDownloadUri(uuid: string): string {
  return `${baseUrl}/api/files/download/${uuid}`
}

export function getThumbnailUri(uuid: string): string {
  return `${baseUrl}/api/files/thumbnail/${uuid}`
}

export function getFiles(uuid?: string): Promise<FsNodesResponse> {
  const path = uuid ? `api/files/${uuid}` : "api/files"
  return fetch(`${baseUrl}/${path}`).then(response => response.json())
}

export function upload(uuid: string, formData: FormData): Promise<UploadResult[]> {
  return fetch(`${baseUrl}/api/files/upload/${uuid}`, {
    method: "POST",
    body: formData
  }).then(response => response.json())
}

export function createDirectory(parent_uuid: string, name: string): Promise<FsNode> {
  return fetch(`${baseUrl}/api/directories`, {
    method: "POST",
    headers: {
      "content-type": "application/json"
    },
    body: JSON.stringify({ parent_uuid, name })
  }).then(response => response.json())
}

export function moveFsNode(source_uuid: string, destination_uuid: string) {
  return fetch(`${baseUrl}/api/files`, {
    method: "PUT",
    headers: {
      "content-type": "application/json"
    },
    body: JSON.stringify({ source_uuid, destination_uuid })
  })
}

export function deleteFsNode(uuid: string): Promise<Response> {
  return fetch(`${baseUrl}/api/files${uuid}`, {
    method: "DELETE"
  })
}
