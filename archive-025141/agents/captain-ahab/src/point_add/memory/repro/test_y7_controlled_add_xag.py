import unittest

import y7_controlled_add_xag as xag


class ControlledAddXagTests(unittest.TestCase):
    def test_two_and_width_two_no_carry_is_unsat(self) -> None:
        report = xag.certificate()
        self.assertEqual(report["inputs"], 5)
        self.assertEqual(report["compatible_first_and_functions"], 4)
        self.assertEqual(report["two_and_witnesses"], 0)
        self.assertTrue(report["two_and_unsat"])


if __name__ == "__main__":
    unittest.main()
