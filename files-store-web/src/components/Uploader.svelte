<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import { upload } from "../FileStoreApi"

  import { fsNodesStore } from "../stores/store"

  export let parent: FsNode
  export let label: string | undefined = "Upload"
  let files

  function uploadFile() {
    const formData = new FormData()
    formData.append("file", files[0])
    upload(parent.uuid, formData).then(fsNodesStore.add).catch(console.error)
  }
</script>

<div class="uploader">
  <input type="file" bind:files on:change="{uploadFile}" />
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
