<script lang="typescript">
  export let name: string
  export let parentUuid: string
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"
  import { createDirectory } from "../FileStoreApi"
  import { fsNodesStore, wantCreateDirectory } from "../stores/store"

  function handleBlur() {
    if (name !== "") {
      createDirectory(parentUuid, name)
        .then(directory => {
          fsNodesStore.addDirectory(directory)
          wantCreateDirectory.update(() => false)
        })
        .catch(error => {
          console.error(error)
          wantCreateDirectory.update(() => false)
        })
    } else {
      wantCreateDirectory.update(() => false)
    }
  }
</script>

<div class="fs-node">
  <div class="icon">
    <DirectoryIcon size="{30}" />
  </div>
  <input autofocus on:blur="{handleBlur}" type="text" bind:value="{name}" />
</div>

<style>
  .fs-node {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
    padding-left: 3px;
    border-left: 3px solid var(--background);
  }

  .icon {
    width: 57px;
    height: 60px;
    padding: 15px 15px 15px 12px;
  }

  input {
    color: var(--primary-text);
  }
</style>
