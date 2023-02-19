<template>
  <el-container class="unselectable">
    <el-main>
      <SystemMonitor/>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import {appWindow} from "@tauri-apps/api/window"
import {listen,TauriEvent} from "@tauri-apps/api/event"
import {onMounted,onUnmounted,reactive} from "vue"
import {invoke} from "@tauri-apps/api/tauri"
import SystemMonitor from './components/SystemMonitor.vue'

function flushTray() {
  invoke<boolean>("update_tray_title",{})
  return flushTray
}

const unListens = reactive({
  blur:()=>{}
})
onMounted(async ()=> {
  unListens.blur = await listen(TauriEvent.WINDOW_BLUR,async (event) => {
    if (event.windowLabel == 'main') {
      appWindow.hide()
    }
  })

  setInterval(flushTray(),5000)
})
onUnmounted(()=> {
  unListens.blur()
})
</script>

<style scoped>
.el-main {
  padding: 8px;
}
</style>