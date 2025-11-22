<script setup>
import { Link } from '@inertiajs/vue3'

defineProps({
  user: Object,
  framework: String,
  phpVersion: String
})
</script>

<template>
  <div class="min-h-screen bg-gray-100 flex items-center justify-center">
    <div class="bg-white p-8 rounded-lg shadow-lg max-w-md w-full">
      <h1 class="text-3xl font-bold text-indigo-600 mb-4">Welcome to {{ framework }}</h1>

      <p class="text-gray-700 mb-4">
        This is a full-stack application using Rust and Vue 3 via Inertia.js.
      </p>

      <div class="bg-gray-50 p-4 rounded border border-gray-200 mb-6">
        <p class="text-sm text-gray-600">Logged in as:</p>
        <p class="font-medium text-gray-900" v-if="user">{{ user.name }}</p>
        <p class="font-medium text-gray-900" v-else>Guest</p>
      </div>

      <div class="flex space-x-4">
        <button class="bg-indigo-600 text-white px-4 py-2 rounded hover:bg-indigo-700 transition">
          Get Started
        </button>
        <Link href="/test" class="text-indigo-600 px-4 py-2 hover:underline flex items-center">
          Test Link
        </Link>
      </div>
    </div>
  </div>
</template>
