<script setup lang="ts">
import { useSettingsStore } from '../stores/settings';
import UiCheckbox from './Checkbox.vue';
const settings = useSettingsStore();
</script>
<template>
  <div>
    <UiCheckbox
      :model-value="settings.saved.clipboard_monitor_enabled"
      :disabled="settings.saving"
      label="返回应用时识别剪贴板链接"
      @update:model-value="settings.saveAppPreferences({ clipboard_monitor_enabled: $event === true })"
    />
    <p class="clipboard-help">仅识别 Bilibili 链接，确认添加后再手动解析。</p>
  </div>
</template>
<style scoped>
.clipboard-help {
  margin: 0;
  font-size: var(--font-12);
  color: var(--color-muted);

}
</style>
