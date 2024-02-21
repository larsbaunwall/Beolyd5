<script setup lang="ts">

</script>

<template>
  <div class="container">
    <router-view v-slot="{ Component }">
      <transition name="slide-up" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped>
.container {
  overflow: hidden;
}
</style>
