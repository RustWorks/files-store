<script lang="typescript">
  import { _ } from "svelte-intl"

  import type { FsNode } from "../FsNode"
  import { wantMoveFsNode, fsNodesStore } from "../stores/store"
  import { Api } from "../services/Api"
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"
  import Button from "./Button.svelte"
  import NavigatorBreadcrumb from "./NavigatorBreadcrumb.svelte"
  import LoaderIcon from "../icons/LoaderIcon.svelte"

  export let fsNode: FsNode
  export let parent: FsNode

  let target: FsNode = parent
  let loading = false

  $: filesResponse = Api.fsNodes.getFiles(target.uuid)

  $: handleMove = () => {
    if (parent.uuid === target.uuid) {
      wantMoveFsNode.set(undefined)
    } else {
      loading = true
      Api.fsNodes
        .moveFsNode(fsNode.uuid, target.uuid)
        .then(() => {
          loading = false
          fsNodesStore.move(fsNode, target)
          wantMoveFsNode.set(undefined)
        })
        .catch(error => {
          loading = false
          // TODO handle error
          console.log(error)
        })
    }
  }
</script>

<div class="move-fs-node-modal">
  {#await filesResponse}
    <div class="navigator">
      <LoaderIcon />
    </div>
  {:then response}
    <div class="navigator">
      <NavigatorBreadcrumb ancestors="{response.ancestors}" onClick="{fsNode => (target = fsNode)}" />
      {#if response.childrens.length === 0}
        <div class="empty">{$_('empty')}</div>
      {/if}
      {#each response.childrens.filter(c => c.node_type === 'directory') as fsNode (fsNode.uuid)}
        <div class="directory" on:click="{() => (target = fsNode)}">
          <DirectoryIcon size="{20}" />
          <div class="directory-name">{fsNode.name}</div>
        </div>
      {/each}
    </div>
  {/await}
  <div class="actions">
    <Button label="{$_('move')}" on:click="{handleMove}" loading="{loading}" />
  </div>
</div>

<style>
  .move-fs-node-modal {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 5px 15px 15px 15px;
    border-top: 1px solid var(--border);
  }

  .navigator {
    overflow-y: auto;
    height: calc(100vh - 30em);
  }

  .directory {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 7px 0;
    cursor: pointer;
    flex-wrap: nowrap;
  }

  .directory-name {
    margin-left: 10px;
    color: var(--primary-text);
    text-decoration: underline;
  }

  .actions {
    display: flex;
    align-items: center;
    padding-top: 10px;
  }

  .empty {
    padding: 15px;
    color: var(--primary-text);
  }
</style>
