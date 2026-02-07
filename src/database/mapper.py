from sqlalchemy import Float, TypeDecorator
from electricity_price_optimizer_py.units import Watt, WattHour, Euro, EuroPerWh


class WattMapper(TypeDecorator):
    impl = Float

    def process_bind_param(self, value: Watt | None, dialect):
        if value is not None:
            return value.get_value()
        return None

    def process_result_value(self, value: float | None, dialect):
        if value is not None:
            return Watt(value)
        return None


class WattHourMapper(TypeDecorator):
    impl = Float

    def process_bind_param(self, value: WattHour | None, dialect):
        if value is not None:
            return value.get_value()
        return None

    def process_result_value(self, value: float | None, dialect):
        if value is not None:
            return WattHour(value)
        return None


class EuroMapper(TypeDecorator):
    impl = Float

    def process_bind_param(self, value: Euro | None, dialect):
        if value is not None:
            return value.get_value()
        return None

    def process_result_value(self, value: float | None, dialect):
        if value is not None:
            return Euro(value)
        return None


class EuroPerWhMapper(TypeDecorator):
    impl = Float

    def process_bind_param(self, value: EuroPerWh | None, dialect):
        if value is not None:
            return value.get_value()
        return None

    def process_result_value(self, value: float | None, dialect):
        if value is not None:
            return EuroPerWh(value)
        return None
