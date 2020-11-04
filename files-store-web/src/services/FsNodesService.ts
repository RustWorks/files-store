import type { CancelTokenSource } from "axios"
import type { FsNode, FsNodeRootType, FsNodesResponse, UploadResult } from "../FsNode"

import { ApiService } from "./ApiService"

export class FsNodesService extends ApiService {
  public getFiles(
    uuid?: string,
    root_type?: FsNodeRootType,
    cancelToken?: CancelTokenSource
  ): Promise<FsNodesResponse> {
    const path = uuid ? `api/fs/${uuid}` : "api/fs"
    const query = root_type ? `?root_type=${root_type}` : ``
    return this.request({
      url: `/${path}${query}`,
      method: "GET",
      cancelToken: cancelToken?.token
    })
  }

  public upload(uuid: string, formData: FormData, cancelToken?: CancelTokenSource): Promise<UploadResult[]> {
    return this.request({
      url: `/api/fs/upload/${uuid}`,
      method: "POST",
      data: formData,
      cancelToken: cancelToken?.token
    })
  }

  public createDirectory(parent_uuid: string, name: string, cancelToken?: CancelTokenSource): Promise<FsNode> {
    return this.request({
      url: `/api/fs/directories`,
      method: "POST",
      data: { parent_uuid, name },
      cancelToken: cancelToken?.token
    })
  }

  public moveFsNode(source_uuid: string, destination_uuid: string, cancelToken?: CancelTokenSource) {
    return this.request({
      url: `/api/fs`,
      method: "PUT",
      data: { source_uuid, destination_uuid },
      cancelToken: cancelToken?.token
    })
  }

  public deleteFsNode(uuid: string, cancelToken?: CancelTokenSource): Promise<Response> {
    return this.request({
      url: `/api/fs/${uuid}`,
      method: "DELETE",
      cancelToken: cancelToken?.token
    })
  }

  public getDownloadUri(uuid: string): string {
    return `${this.config.baseURL}/api/fs/download/${uuid}?access_token=${this.storage.getItemSync(
      this.STORAGE_AUTH_TOKEN_KEY
    )}`
  }

  public getThumbnailUri(uuid: string): string {
    return `${this.config.baseURL}/api/fs/thumbnail/${uuid}?access_token=${this.storage.getItemSync(
      this.STORAGE_AUTH_TOKEN_KEY
    )}`
  }
}
