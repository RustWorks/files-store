<script lang="typescript">
  import type { FsNode } from "../FsNode"
  import HomeIcon from "../icons/HomeIcon.svelte"

  export let ancestors: FsNode[]
</script>

<div class="breadcrumb">
  {#each ancestors as ancestor (ancestor.uuid)}
    {#if ancestor.node_type === 'root'}
      <a href="#/">
        <HomeIcon size="{30}" />
      </a>
    {:else}
      <div class="separaton">/</div>
      <a class="crumb" href="#/directory/{ancestor.uuid}">{ancestor.name}</a>
    {/if}
  {/each}
</div>

<style>
  .breadcrumb {
    padding: 15px;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .separaton {
    color: var(--primary);
    margin-left: 5px;
    margin-right: 5px;
  }

  .crumb {
    color: var(--primary);
    text-decoration: none;
  }

  .crumb:hover {
    text-decoration: underline;
  }
</style>
