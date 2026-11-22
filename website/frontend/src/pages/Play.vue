<script setup lang="ts">
import { ref } from 'vue'
import { ApiError, uploadBot } from '../api'
import iconSuccess from '../assets/images/icon-success.svg'
import iconSubmit from '../assets/images/icon-submit.svg'

const fileInput = ref<HTMLInputElement | null>(null)
const selected = ref<File | null>(null)
const uploading = ref(false)
const error = ref('')
const success = ref(false)

function formatSize(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`
  }
  if (bytes < 1024 * 1024) {
    return `${(bytes / 1024).toFixed(1)} KB`
  }
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function onFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0] ?? null

  error.value = ''
  success.value = false

  if (file && !file.name.toLowerCase().endsWith('.zip')) {
    selected.value = null
    error.value = 'Please choose a .zip archive containing your bot.'
    target.value = ''
    return
  }

  selected.value = file
}

function chooseAnother() {
  selected.value = null
  error.value = ''
  success.value = false
  if (fileInput.value) {
    fileInput.value.value = ''
    fileInput.value.click()
  }
}

async function submit() {
  if (!selected.value || uploading.value) {
    return
  }

  error.value = ''
  uploading.value = true

  try {
    await uploadBot(selected.value)
    success.value = true
    selected.value = null
  } catch (err) {
    error.value =
      err instanceof ApiError ? err.message : 'Upload failed. Please check your connection and try again.'
  } finally {
    uploading.value = false
  }
}

function reset() {
  selected.value = null
  error.value = ''
  success.value = false
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}
</script>

<template>
  <div class="container play-container">
    <div class="page-header">
      <h1>Submit a Bot</h1>
    </div>

    <p class="page-description">
      Upload the .zip archive produced by your starter kit. Your bot will be matched against other
      bots once real submissions are enabled.
    </p>

    <div class="play-upload-section">
      <div class="upload-container">
        <div v-if="success" class="upload-state">
          <img class="upload-zone-icon" :src="iconSuccess" alt="" />
          <h2>Upload received</h2>
          <p class="upload-state-desc">
            Your archive reached the server. Storage is not implemented yet, so it is not kept and
            no new bot entry is created.
          </p>
          <div class="upload-state-buttons">
            <button type="button" class="btn-ha" @click="reset">Upload another bot</button>
            <RouterLink class="btn-ha btn-ha-clear" to="/my-bots">View my bots</RouterLink>
          </div>
        </div>

        <div v-else-if="selected" class="upload-state">
          <h2>Ready to submit</h2>
          <p class="upload-state-filename">{{ selected.name }}</p>
          <p class="upload-state-desc">
            {{ formatSize(selected.size) }} · Make sure the archive contains your bot source and
            build files.
          </p>

          <p v-if="error" class="upload-state-error" role="alert">{{ error }}</p>

          <div class="upload-state-buttons">
            <button
              type="button"
              class="btn-ha btn-ha-clear"
              :disabled="uploading"
              @click="chooseAnother"
            >
              Choose another file
            </button>
            <button type="button" class="btn-ha" :disabled="uploading" @click="submit">
              {{ uploading ? 'Uploading…' : 'Submit bot' }}
            </button>
          </div>
        </div>

        <div v-else>
          <h2>Upload your bot</h2>

          <label class="upload-zone" for="bot-file">
            <img class="upload-zone-icon" :src="iconSubmit" alt="" />
            <span class="upload-zone-desc">Choose a .zip archive</span>
            <span class="upload-zone-caption">Click to browse for your bot package</span>
          </label>

          <p v-if="error" class="upload-state-error" role="alert">{{ error }}</p>
        </div>

        <input
          id="bot-file"
          ref="fileInput"
          class="file-input"
          type="file"
          accept=".zip"
          @change="onFileChange"
        />
      </div>
    </div>

    <p class="upload-note">
      This page will later host real submissions: persistent storage, language detection, and a
      processing queue. For now the backend only acknowledges the upload.
    </p>
  </div>
</template>

<style scoped>
.file-input {
  position: absolute;
  width: 1px;
  height: 1px;
  margin: -1px;
  padding: 0;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.upload-zone {
  display: block;
  padding: 36px 20px;
  border: 2px dashed #474951;
  border-radius: 12px;
  cursor: pointer;
  transition: border-color 0.15s ease, background-color 0.15s ease;
}

.upload-zone:hover,
.upload-zone:focus-within {
  border-color: #ffbe00;
  background-color: rgba(255, 190, 0, 0.04);
}

.upload-zone-desc {
  display: block;
  color: #ecfffb;
  font-family: 'Teko', 'Arial Narrow', sans-serif;
  font-size: 2.4rem;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.upload-zone-caption {
  display: block;
  color: #b9b8b8;
  font-size: 1.4rem;
}

.upload-zone-icon {
  width: 54px;
  height: 54px;
  object-fit: contain;
}

.upload-state-buttons {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 14px;
}

.upload-state-error {
  color: #ff2d55;
}

@media (max-width: 767px) {
  .upload-state-buttons :deep(.btn-ha) {
    width: 100%;
  }
}
</style>
