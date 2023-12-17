import { FormEvent } from 'react'
import { cn } from '@/lib/utils'

export interface ButtonWithIconProps {
  className?: string
  text: string
  type: 'submit' | 'button' | 'reset'
  icon: string
  color?: 'info' | 'warn'
  onClick?: () => void
}

const ButtonWithIcon = ({
  className,
  text,
  type,
  icon,
  color,
  onClick,
}: ButtonWithIconProps) => {
  let colorName = ''
  switch (color) {
    case 'info':
      colorName = 'dark:bg-blue-900'
      break
    case 'warn':
      colorName = 'dark:bg-red-900'
      break
    default:
      colorName = 'dark:bg-blue-900'
      break
  }

  return (
    <button
      className={cn('px-2 rounded-full', colorName, className)}
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
