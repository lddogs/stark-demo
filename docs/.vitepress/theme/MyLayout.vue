<script setup>
import DefaultTheme from 'vitepress/theme'
import { onMounted, watch, nextTick } from 'vue'
import { useRoute } from 'vitepress'

const { Layout } = DefaultTheme

const route = useRoute()

function renderMath() {
  if (typeof window !== 'undefined' && window.katex) {
    const mathElements = document.getElementsByClassName('math')
    Array.from(mathElements).forEach((el) => {
      katex.render(el.textContent, el, {
        throwOnError: false,
        displayMode: el.classList.contains('display')
      })
    })
  }
}

onMounted(() => {
  renderMath()
})

watch(() => route.path, () => {
  nextTick(() => {
    renderMath()
  })
})
</script>

<template>
  <Layout>
    <template #layout-top>
      <!-- You can add custom content here if needed -->
    </template>
  </Layout>
</template>