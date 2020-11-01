<script lang="typescript">
  import { _ } from "svelte-intl"
  import { navigate } from "@bjornlu/svelte-router"

  import { Api } from "../services/Api"
  import Button from "../components/Button.svelte"
  import type { ApiErrors } from "../services/ApiError"
  import { getFieldErrorMessage } from "../utils"

  let username: string = ""
  let password: string = ""
  let loading: boolean = false
  let loginErrors: ApiErrors | undefined = undefined

  function submit() {
    loading = true
    Api.auth
      .login({ username, password })
      .then(() => {
        loading = false
        navigate("#/")
      })
      .catch((errors: ApiErrors) => {
        loginErrors = errors
        loading = false
      })
  }
</script>

<main class="main">
  <div class="form">
    <div class="form-field">
      <input type="email" bind:value="{username}" placeholder="{$_('email')}" />
      {#if loginErrors}
        <div class="error">{getFieldErrorMessage('username', (k, p) => $_(k, p), loginErrors)}</div>
      {/if}
    </div>

    <div class="form-field">
      <input type="password" bind:value="{password}" placeholder="{$_('password')}" />
      {#if loginErrors}
        <div class="error">{getFieldErrorMessage('password', (k, p) => $_(k, p), loginErrors)}</div>
      {/if}
    </div>
    <Button label="Login" loading="{loading}" on:click="{submit}" />
  </div>
</main>

<style>
  input {
    width: 100%;
    padding: 17px 15px;
    font-size: 1rem;
    border-radius: 3px;
    border: 1px solid var(--border);
  }

  .form-field {
    margin-bottom: 20px;
  }

  .error {
    padding: 5px 0;
    color: rgb(145, 26, 26);
  }

  .form {
    width: 100%;
    padding: 20px;
    display: flex;
    flex-direction: column;
  }

  .main {
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: nowrap;
  }
  @media screen and (min-width: 500px) {
    .form {
      width: 400px;
    }
  }
</style>
