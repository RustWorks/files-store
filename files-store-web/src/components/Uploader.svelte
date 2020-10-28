<script lang="typescript">
  import type { FsNode, UploadResult } from "../FsNode"
  import { upload } from "../FileStoreApi"
  import { fsNodesStore } from "../stores/store"

  export let parent: FsNode
  export let label: string | undefined = "Upload"

  let files: FileList

  export function isFsNode(value: UploadResult): value is FsNode {
    return !!(value as FsNode).uuid
  }

  function uploadFile() {
    const formData = new FormData()
    for (var i = 0; i < files.length; i++) {
      formData.append(`file-${i}`, files[i])
    }
    upload(parent.uuid, formData)
      .then(uploaded => {
        fsNodesStore.add(uploaded.filter(isFsNode))
      })
      .catch(console.error)
  }
</script>

<div class="uploader">
  <input type="file" multiple bind:files on:change="{uploadFile}" />
  <label for="upload-input">{label}</label>
</div>

<style>
  .uploader {
    min-width: 100px;
    position: relative;
    height: 35px;
    padding-left: 15px;
    padding-right: 15px;
    background-color: var(--primary);
    border-radius: 3px;
    text-align: center;
    display: inline-flex;
    justify-content: center;
    overflow: hidden;
    margin-right: 10px;
  }

  .uploader:hover {
    opacity: 0.8;
  }

  input {
    position: absolute;
    opacity: 0;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    cursor: pointer;
  }

  label {
    color: var(--background);
    font-size: 0.9rem;
    height: 35px;
    padding-left: 15px;
    padding-right: 15px;
    font-weight: bold;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: all 0.2s;
    cursor: pointer;
  }
</style>
