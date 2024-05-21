import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('CT/TC1_Accesstoken'), [:], FailureHandling.STOP_ON_FAILURE)

cartresponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Create Cart', [('host') : GlobalVariable.host, ('project-key') : 'kv-qa-team']))

WS.verifyResponseStatusCode(cartresponse, 201, FailureHandling.STOP_ON_FAILURE)

// cardid & cartversion  to GlobalVariable

cartid = WS.getElementPropertyValue(cartresponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(cartresponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//Addlineitem request
lineitemresponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddLineItem', [('host') : GlobalVariable.host
            , ('project-key') : 'kv-qa-team']))

WS.verifyResponseStatusCode(lineitemresponse, 200, FailureHandling.STOP_ON_FAILURE)

// update cartid & cartversion

cartid = WS.getElementPropertyValue(lineitemresponse, 'id')

GlobalVariable.cart_id = cartid

cartversion = WS.getElementPropertyValue(lineitemresponse, 'version')

GlobalVariable.cart_version = cartversion

println(GlobalVariable.cart_id)

println(GlobalVariable.cart_version)

//update cartotal $ totallineitem  to GlobalVariable
Carttotal= WS.getElementPropertyValue(lineitemresponse, 'totalPrice.centAmount')
println("totalprice before discount"+""+ Carttotal)

totalLineItemQuantity=WS.getElementPropertyValue(lineitemresponse, 'totalLineItemQuantity')
println("totallineitem before discount"+""+ totalLineItemQuantity)

GlobalVariable.Cart_total=Carttotal
GlobalVariable.total_LineItem_Quantity=totalLineItemQuantity

//Check product got added in cart
if (Carttotal >0) {
	println("Product got added in the cart"+Carttotal)
}
else {
	println("Product didnot get added in the cart"+Carttotal)
}
	
//specific cart condition

if (Carttotal >= 2000) {
	println("Satisfy the cart condition"+Carttotal)
}
else {
	println("Do not satisfy the cart condition"+Carttotal)
}
	


 



