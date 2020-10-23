<script lang="typescript">
  import LoaderIcon from "../icons/LoaderIcon.svelte"

  export let label: string
  export let loading: boolean = false
</script>

<button on:click>
  {#if loading}
    <LoaderIcon size="{30}" />
  {:else}{label}{/if}
</button>

<style>
  button {
    appearance: none;
    border: none;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    min-width: 100px;
    height: 35px;
    padding-left: 15px;
    padding-right: 15px;
    font-size: 0.9rem;
    font-weight: bold;
    transition: all 0.2s;
    cursor: pointer;
    color: var(--background);
    background-color: var(--primary);
    border-radius: 3px;
    text-align: center;
  }

  button:hover {
    opacity: 0.8;
  }
</style>
