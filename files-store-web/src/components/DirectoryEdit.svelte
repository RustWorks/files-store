<script lang="typescript">
  import { createDirectory } from "../FileStoreApi"
  import { fsNodesStore, wantCreateDirectory } from "../stores/store"
  import DirectoryIcon from "../icons/DirectoryIcon.svelte"

  export let name: string
  export let parentUuid: string

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
  <!-- svelte-ignore a11y-autofocus -->
  <input autofocus on:blur="{handleBlur}" type="text" bind:value="{name}" />
</div>

<style>
  .fs-node {
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
  }

  .icon {
    width: 57px;
    height: 60px;
    padding: 15px 15px 15px 11px;
    border-left: 4px solid var(--background);
  }

  input {
    color: var(--primary-text);
  }
</style>
