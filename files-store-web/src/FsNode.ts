export type FsNodeRootType = "root" | "bin"

export type FsNodeType = "directory" | "file" | FsNodeRootType

export interface FileMetadata {
  type: "File"
  content_type: string
  hash: string
  size: number
}

export type DirectoryMetadata = {
  type: "Directory"
}

export interface ThumbnailMetadata {
  type: "Thumbnail"
  content_type: string
}

export type FsNodeMetadata = FileMetadata | DirectoryMetadata | ThumbnailMetadata

export interface FsNode {
  uuid: string
  node_type: FsNodeType
  name: string
  metadata: FsNodeMetadata
  user_uuid: string
  created_at: string
  updated_at: string
}

export interface FsNodesResponse {
  parent: FsNode
  childrens: FsNode[]
  ancestors: FsNode[]
}

export interface UploadError {
  error_message: string
  filename: string
}

export type UploadResult = UploadError | FsNode
