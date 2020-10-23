<script lang="typescript">
  export let selected: boolean
  // import { fly } from "svelte/transition"
  // import { quintOut } from "svelte/easing"
</script>

<!-- <div class="flaying-container" transition:fly="{{ duration: 200, y: 200, easing: quintOut }}" /> -->
<div class="flaying-container" class:selected="{!selected}">
  <div role="button" class="close" on:click></div>
  <div class="content">
    <slot />
  </div>
</div>

<style>
  .flaying-container {
    grid-area: side-panel;
    border-top: 1px solid var(--divider);
    border-top-left-radius: 15px;
    border-top-right-radius: 15px;
    box-shadow: 0px 0px 3px 0px rgba(0, 0, 0, 0.2);
    height: 40vh;
  }

  .selected {
    display: none;
  }

  .content {
    padding: 0 20px;
  }
  .close {
    position: relative;
    width: 100%;
    height: 20px;
    display: flex;
    justify-content: center;
    align-items: center;
    border-top-left-radius: 15px;
    border-top-right-radius: 15px;
    border: none;
    appearance: none;
    cursor: pointer;
  }
  .close::after {
    content: "";
    height: 3px;
    width: 50px;
    background-color: rgba(0, 0, 0, 0.2);
  }
  .close:hover::after {
    background-color: rgba(0, 0, 0, 0.4);
  }

  @media screen and (min-width: 700px) {
    .flaying-container {
      width: 300px;
      height: auto;
      border-radius: 0px;
      overflow-y: auto;
    }
    .close {
      justify-content: end;
      padding-right: 15px;
      border-top-left-radius: 0;
      border-top-right-radius: 0;
    }
    .close::after {
      width: 15px;
    }

    .selected {
      display: block;
    }
  }
</style>
