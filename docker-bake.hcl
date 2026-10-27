variable "TAG" {
  default = "local"
}

variable "ARCH_SUFFIX" {
  default = ""
}

variable "REGISTRY_PREFIX" {
  default = "ghcr.io/alisterd51/rusty"
}

variable "BACKEND_URL" {
  default = "https://rusty.anclarma.fr"
}
variable "FRONTEND_URL" {
  default = "https://rusty.anclarma.fr"
}
variable "AUTH_URL" {
  default = "https://auth.rusty.anclarma.fr"
}

group "default" {
  targets = ["rusty"]
}

target "rusty" {
  name = item
  matrix = {
    item = [
      "crud-server",
      "game-server",
      "game-frontend",
      "migration",
      "rauthy-init"
    ]
  }
  context    = "."
  dockerfile = "Containerfile"
  target     = "runtime-${item}"
  tags = [
    "${REGISTRY_PREFIX}-${item}:${TAG}${ARCH_SUFFIX}"
  ]
  args = {
    BACKEND_URL  = "${BACKEND_URL}"
    FRONTEND_URL = "${FRONTEND_URL}"
    AUTH_URL     = "${AUTH_URL}"
  }
}
