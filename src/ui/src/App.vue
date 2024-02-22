<script setup lang="ts">

</script>

<template>
  <div class="container">
    <router-view v-slot="{ Component }">
        <component :is="Component" />
    </router-view>
  </div>
</template>

<style scoped>
.container {
  overflow: hidden;
}
</style>
