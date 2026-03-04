<script setup lang="ts">
defineProps<{
  heading: string;
  hint: string;
  code: string;
}>();
</script>

<template>
  <div class="hero-shell-panel" role="region" :aria-label="heading">
    <div class="hero-shell-head">
      <span class="hero-shell-dot hero-shell-dot-red" aria-hidden="true" />
      <span class="hero-shell-dot hero-shell-dot-yellow" aria-hidden="true" />
      <span class="hero-shell-dot hero-shell-dot-green" aria-hidden="true" />
      <span class="hero-shell-title">{{ heading }}</span>
    </div>
    <p class="hero-shell-hint">{{ hint }}</p>
    <pre class="hero-shell-code"><code>{{ code }}</code></pre>
  </div>
</template>
