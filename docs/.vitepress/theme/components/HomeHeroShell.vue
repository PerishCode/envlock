<script setup lang="ts">
import { ref } from "vue";

const commandBlock = [
  "curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh",
  "eval \"$(envlock)\"",
  "echo \"$ENVLOCK_PROFILE\""
].join("\n");

const copied = ref(false);

async function copyCommands() {
  try {
    await navigator.clipboard.writeText(commandBlock);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 1500);
  } catch {
    copied.value = false;
  }
}
</script>

<template>
  <div class="hero-shell-panel" role="img" aria-label="envlock cold start shell commands">
    <div class="hero-shell-head">
      <span class="dot dot-red" />
      <span class="dot dot-yellow" />
      <span class="dot dot-green" />
      <span class="hero-shell-title">envlock quick verify</span>
      <button class="hero-shell-copy" type="button" @click="copyCommands">
        {{ copied ? "Copied" : "Copy" }}
      </button>
    </div>
    <pre class="hero-shell-code"><code>$ curl -fsSL https://raw.githubusercontent.com/PerishCode/envlock/main/scripts/install.sh | sh
$ eval "$(envlock)"
$ echo "$ENVLOCK_PROFILE"
default</code></pre>
  </div>
</template>
