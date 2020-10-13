export type FsNodeType = "directory" | "file" | "root"

export type FileMetadata = {
  content_type: string,
  hash: string,
  size: number
}

export type DirectoryMetadata = {}

export interface FsNode {
    uuid: string,
    node_type: FsNodeType,
    name: string,
    metadata: FileMetadata | DirectoryMetadata,
    user_uuid: string,
    created_at: string,
    updated_at: string
}
