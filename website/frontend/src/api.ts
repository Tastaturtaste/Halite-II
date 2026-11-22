/**
 * Typed wrappers around the Halite II backend API.
 * The backend session is an HttpOnly cookie, so every request uses
 * `credentials: 'same-origin'`.
 */

export interface LoginResponse {
  email: string
}

export interface MeResponse {
  email: string
}

export interface StarterKit {
  name: string
  language: string
  downloadUrl: string
}

export interface StarterKitsResponse {
  starterkits: StarterKit[]
}

export interface UploadBotResponse {
  ok: boolean
}

export interface Bot {
  id: number
  name: string
  language: string
  rank: number
  gamesPlayed: number
  uploadDate: string
}

export interface BotsResponse {
  bots: Bot[]
}

export interface GameSummary {
  id: number
  name: string
}

export interface GamesResponse {
  games: GameSummary[]
}

export interface GameDetail {
  id: number
  mapWidth: number
  mapHeight: number
  players: string[]
  turns: number
}

export class ApiError extends Error {
  status: number

  constructor(status: number, message: string) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

const API_BASE = '/api'

async function request<T>(path: string, init: RequestInit = {}): Promise<T> {
  const headers = new Headers(init.headers)
  if (init.body !== undefined && !(init.body instanceof FormData)) {
    headers.set('Content-Type', 'application/json')
  }

  const response = await fetch(`${API_BASE}${path}`, {
    ...init,
    headers,
    credentials: 'same-origin',
  })

  if (!response.ok) {
    let message = response.statusText
    try {
      const body = (await response.json()) as { error?: string; message?: string }
      message = body.error ?? body.message ?? message
    } catch {
      // Response had no JSON body; keep the status text.
    }
    throw new ApiError(response.status, message || `Request failed (${response.status})`)
  }

  if (response.status === 204) {
    return undefined as T
  }

  return (await response.json()) as T
}

export function login(email: string): Promise<LoginResponse> {
  return request<LoginResponse>('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email }),
  })
}

export function fetchMe(): Promise<MeResponse> {
  return request<MeResponse>('/me')
}

export function logout(): Promise<void> {
  return request<void>('/logout', { method: 'POST' })
}

export function fetchStarterKits(): Promise<StarterKitsResponse> {
  return request<StarterKitsResponse>('/starterkits')
}

export function uploadBot(file: File): Promise<UploadBotResponse> {
  const body = new FormData()
  body.append('bot', file)
  return request<UploadBotResponse>('/bots/upload', { method: 'POST', body })
}

export function fetchMyBots(): Promise<BotsResponse> {
  return request<BotsResponse>('/users/me/bots')
}

export function fetchGames(): Promise<GamesResponse> {
  return request<GamesResponse>('/games')
}

export function fetchGame(id: number | string): Promise<GameDetail> {
  return request<GameDetail>(`/games/${encodeURIComponent(String(id))}`)
}
