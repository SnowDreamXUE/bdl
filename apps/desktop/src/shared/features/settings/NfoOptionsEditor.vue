<script setup lang="ts">
import { computed, ref } from 'vue'
import type { NfoExtraFields, NfoOptions } from '../../api/dto'
import SettingsCard from './SettingsCard.vue'
import UiCheckbox from '../../ui/Checkbox.vue'
import UiTextField from '../../ui/TextField.vue'
import UiSelect from '../../ui/Select.vue'
import UiButton from '../../ui/Button.vue'
import NamingTemplateField from '../../ui/NamingTemplateField.vue'
import { applyNfoLayout, nfoExtraDefinitions, nfoTemplateError, nfoVariables, normalizeNfoOptions, previewNfoTemplate } from '../../utils/nfo'

const model = defineModel<NfoOptions>({ required: true })
defineProps<{ disabled?: boolean }>()
const options = computed(() => normalizeNfoOptions(model.value))
const layout = ref('standard')
const fields = [
  ['include_plot', '写入简介'], ['include_date', '写入发布日期与年份'],
  ['include_runtime', '写入时长'], ['include_genre', '使用 B 站分区作为分类'],
  ['include_source', '写入 B 站唯一标识'], ['include_cover', '关联已保存的封面'],
  ['uploader_as_actor', '将 UP 主加入人物列表'],
] as const
const title = computed(() => previewNfoTemplate(options.value.title_template))
const plot = computed(() => previewNfoTemplate(options.value.plot_template))
const extraPreview = computed(() => nfoExtraDefinitions.map(field => ({ ...field, value: previewNfoTemplate(options.value.extra[field.key]) })).filter(field => field.value.trim()))
function update<K extends keyof NfoOptions>(key: K, value: NfoOptions[K]) {
  model.value = { ...options.value, [key]: value }
}
function updateExtra(key: keyof NfoExtraFields, value: string) {
  update('extra', { ...options.value.extra, [key]: value })
}
</script>

<template>
  <SettingsCard title="NFO 媒体信息">
    <p class="settings-note">给媒体库保存视频资料卡。默认可直接使用；变量会在下载时替换成实际信息，不改变文件名。</p>
    <details>
      <summary class="settings-note">套用展示方案</summary>
      <div class="nfo-section">
        <UiSelect
          v-model="layout" label="NFO 展示方案" :disabled="disabled" :options="[
          { value: 'standard', label: '标准资料卡' }, { value: 'uploader', label: '按 UP 主归类' }, { value: 'multipart', label: '多 P 视频归组' },
        ]" />
        <p class="settings-note">套用会替换标题、简介与进阶字段，保留字段开关和固定标签。作品集用于归类独立视频，不生成电视剧季集。</p>
        <UiButton variant="secondary" :disabled="disabled" @click="model = applyNfoLayout(options, layout)">套用方案</UiButton>
      </div>
    </details>
    <NamingTemplateField
      :model-value="options.title_template" label="NFO 标题模板" :variables="nfoVariables" :disabled="disabled"
      :error="!options.title_template.trim() ? '标题模板不能为空' : nfoTemplateError(options.title_template) ?? undefined"
      @update:model-value="update('title_template', $event)" />
    <p class="settings-note">标题示例：{{ title }}</p>
    <div class="grid gap-3 sm:grid-cols-2">
      <UiCheckbox v-for="[key, label] in fields" :key="key" :model-value="options[key]" :label="label" :disabled="disabled" @update:model-value="update(key, $event === true)" />
    </div>
    <NamingTemplateField
      v-if="options.include_plot" :model-value="options.plot_template" label="NFO 简介模板" multiline :variables="nfoVariables" :disabled="disabled"
      :error="nfoTemplateError(options.plot_template) ?? undefined" @update:model-value="update('plot_template', $event)" />
    <UiTextField :model-value="options.custom_tags" label="自定义标签" helper="固定标签用逗号分隔，例如：教程, 收藏。可使用英文或中文逗号。" placeholder="教程, 收藏" :disabled="disabled" @update:model-value="update('custom_tags', $event)" />
    <details>
      <summary class="settings-note">进阶字段：排序、作品集与制作信息</summary>
      <div class="nfo-section">
        <p class="settings-note">全部可留空。也可以直接输入文字，或点击加号插入变量。制作信息只填写已知事实，同一预设会应用到所有使用它的任务。</p>
        <NamingTemplateField
          v-for="field in nfoExtraDefinitions" :key="field.key" :model-value="options.extra[field.key]"
          :label="field.label" :helper="field.helper" :variables="nfoVariables" :disabled="disabled"
          :error="nfoTemplateError(options.extra[field.key]) ?? undefined" @update:model-value="updateExtra(field.key, $event)" />
      </div>
    </details>
    <details>
      <summary class="settings-note">模板变量与示例</summary>
      <div class="nfo-section">
        <p class="settings-note">各输入框旁的加号可插入变量，中文说明与文件命名一致。示例使用虚拟数据；实际缺失的字段会省略。</p>
        <p class="settings-note">标题示例：{{ title }}</p>
        <p v-if="options.include_plot" class="settings-note nfo-plot-preview">简介示例：{{ plot }}</p>
        <p v-for="field in extraPreview" :key="field.key" class="settings-note nfo-plot-preview">{{ field.label }}：{{ field.value }}</p>
        <p class="settings-note">日期文字支持年-月-日、年.月.日和紧凑格式；序号可用 :02、:03、:04 补零。媒体库的发布日期始终使用标准年-月-日。</p>
      </div>
    </details>
    <p class="settings-note">封面需同时下载并独立保存。UP 主人物仅便于筛选。播放器可能只显示部分字段；评分、观看进度和实际分辨率由媒体库管理。</p>
  </SettingsCard>
</template>

<style scoped>
.nfo-section {
  display: grid;
  gap: var(--space-12);
  padding-top: var(--space-8);
  min-width: 0;
}

.nfo-plot-preview {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
</style>
