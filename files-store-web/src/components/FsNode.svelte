<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import { getDownloadUri, getThumbnailUri } from "../FileStoreApi"
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"
  import FileIcon from "../icons/FileIcon.svelte"
  import { selectedFsNode } from "../stores/store"
  export let fsNode: FsNode
  let selected: boolean = false
  selectedFsNode.subscribe(nodes => {
    selected = !!nodes.find(n => n.uuid === fsNode.uuid)
  })

  let href = fsNode.node_type === "directory" ? `#/directory/${fsNode.uuid}` : getDownloadUri(fsNode.uuid)
</script>

<div class="fs-node">
  <div class="icon" class:selected on:click="{() => selectedFsNode.toggle(fsNode)}">
    {#if fsNode.node_type === 'directory'}
      <DirectoryIcon size="{30}" />
    {:else if fsNode.node_type === 'file' && fsNode.metadata.type === 'File' && (fsNode.metadata.content_type === 'image/jpeg' || fsNode.metadata.content_type === 'image/png')}
      <img class="thumbnail" src="{getThumbnailUri(fsNode.uuid)}" alt="thumbnail" />
    {:else}
      <FileIcon size="{30}" />
    {/if}
  </div>
  <a class="name" href="{href}">{fsNode.name}</a>
</div>

<style>
  .fs-node {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
  }

  .thumbnail {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .icon {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 57px;
    height: 60px;
    padding: 10px 10px 10px 8px;
    cursor: pointer;
    border-left: 4px solid var(--background);
  }

  .icon:hover {
    background-color: rgba(151, 151, 151, 0.1);
    border-left: 4px solid #dadada2c;
  }

  .selected {
    border-left: 4px solid var(--primary);
    background-color: rgba(201, 201, 201, 0.1);
  }

  .selected:hover {
    border-left: 4px solid var(--primary);
  }

  .name {
    padding-right: 15px;
    text-decoration: none;
    color: var(--primary-text);
    word-break: break-all;
    overflow: hidden;
  }

  .name:hover {
    text-decoration: underline;
  }
</style>
