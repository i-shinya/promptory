import { FormEvent } from 'react'

export interface ButtonWithIconProps {
  text: string
  type: 'submit' | 'button' | 'reset'
  icon: string
  color?: 'info' | 'warn'
  onClick?: () => void
}

const ButtonWithIcon = ({
  text,
  type,
  icon,
  color,
  onClick,
}: ButtonWithIconProps) => {
  let colorClass = ''
  switch (color) {
    case 'info':
      colorClass = 'dark:bg-blue-900 dark:text-white'
      break
    case 'warn':
      colorClass = 'dark:bg-red-900 dark:text-white'
      break
    default:
      colorClass = 'dark:bg-blue-900 dark:text-white'
      break
  }

  return (
    <button
      className={`px-2 rounded-full ${colorClass}`}
      type={type}
      onClick={
        onClick
          ? (e: FormEvent) => {
              e.stopPropagation()
              e.preventDefault()
              onClick()
            }
          : undefined
      }
    >
      <div className="flex items-center">
        <span className={`${icon} mr-1`}></span>
        <span>{text}</span>
      </div>
    </button>
  )
}

export default ButtonWithIcon
