export interface RunSettings {
  model: string
  temperature: number
  userPrompt: string
}

export interface ComparingPromtpRow {
  id: number
  systemPrompt: string
  answer: string
}
