<script setup lang="ts">
defineProps<{
  heading: string;
  hint: string;
  title: string;
  code: string;
}>();
</script>

<template>
  <div class="hero-shell-panel" role="region" :aria-label="heading">
    <div class="hero-shell-top">
      <h2>{{ heading }}</h2>
      <p class="hero-shell-hint">{{ hint }}</p>
    </div>
    <div class="hero-shell-head">
      <span class="hero-shell-dot hero-shell-dot-red" aria-hidden="true" />
      <span class="hero-shell-dot hero-shell-dot-yellow" aria-hidden="true" />
      <span class="hero-shell-dot hero-shell-dot-green" aria-hidden="true" />
      <span class="hero-shell-title">{{ title }}</span>
    </div>
    <pre class="hero-shell-code"><code>{{ code }}</code></pre>
  </div>
</template>
