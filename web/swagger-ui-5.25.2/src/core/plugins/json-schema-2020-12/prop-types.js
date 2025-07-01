/**
 * @prettier
 */
import PropTypes from "web/spec/swagger-ui-5.25.2/src/core/plugins/json-schema-2020-12/prop-types"

export const objectSchema = PropTypes.object

export const booleanSchema = PropTypes.bool

export const schema = PropTypes.oneOfType([objectSchema, booleanSchema])
