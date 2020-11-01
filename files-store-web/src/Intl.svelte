<script context="module">
  import { locale, translations, getBrowserLocale } from "svelte-intl"

  translations.update({
    fr: {
      title: "Files Store",
      bin: "Corbeille",
      empty: "Vide",
      move: "Déplacer",
      delete: "Supprimer",
      createDirectory: "Créer un dossier",
      upload: "Upload",
      email: "Email",
      password: "Mot de passe",
      "validation.email": "Email invalide",
      "validation.login.short": "Login doit être supérieure à {min}",
      "validation.login.long": " Login doit être inférieure à {max}",
      "validation.password.short": "Password doit être supérieure à {min}",
      "validation.error": "Erreurs de validations"
    },
    en: {
      title: "Files Store",
      bin: "Bin",
      empty: "Empty",
      move: "Move",
      delete: "Delete",
      createDirectory: "Create Directory",
      upload: "Upload",
      email: "Email",
      password: "Password",
      "validation.email": "Invalid email",
      "validation.login.short": "Login should be greater than {min}",
      "validation.login.long": "Login should be less than {max}",
      "validation.password.short": "Password should be greater than {min}",
      "validation.error": "Validations errors"
    }
  })

  locale.set(getBrowserLocale("fr"))
</script>
