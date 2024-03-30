import { verify_receipt_buffer } from './pkg';
import receipt from '../testing.receipt';

console.log("About to verify...");
console.log(verify_receipt_buffer(receipt));