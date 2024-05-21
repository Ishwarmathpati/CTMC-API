<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddDiscountCode</name>
   <tag></tag>
   <elementGuidId>af14685f-94d2-4d48-85dc-100400cf1dff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>pW3L9ZlT78HLaPm4smMbh2Cnw6ZVU0Em</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${GlobalVariable.cart_version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addDiscountCode\&quot;,\n            \&quot;code\&quot; : \&quot;PROD123\&quot;\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bb1b2adb-8cab-4bd2-bd87-336095239673</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.ctp_access_token}</value>
      <webElementGuid>e876d729-e89a-40b2-b4f4-2f816c7943a3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/${GlobalVariable.project_key}/carts/${GlobalVariable.cart_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>00683e91-6415-4fa7-8502-bd205d1313f9</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project_key</defaultValue>
      <description></description>
      <id>faa417ff-15d6-4eec-ad26-6b5fb451d394</id>
      <masked>false</masked>
      <name>project_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_id</defaultValue>
      <description></description>
      <id>6fbebd8a-70ea-47c0-8081-4b5481ec2970</id>
      <masked>false</masked>
      <name>cart_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_version</defaultValue>
      <description></description>
      <id>7cac3b9a-bb90-43ac-821f-03f2ca598c78</id>
      <masked>false</masked>
      <name>cart_version</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'lineItems[0].discountedPrice.value.centAmount', 16000)
WS.verifyElementPropertyValue(response, 'lineItems[0].price.value.centAmount', 20000)
WS.verifyElementPropertyValue(response, 'lineItems[0].productKey', &quot;sipper-coffee-mug&quot;)
WS.verifyElementPropertyValue(response, 'lineItems[0].discountedPricePerQuantity[0].discountedPrice.includedDiscounts[0].discountedAmount.fractionDigits', 4000)
WS.verifyElementPropertyValue(response, 'lineItems[0].discountedPrice.includedDiscounts[0].discountedAmount.centAmount', &quot;MXN&quot;)
WS.verifyElementPropertyValue(response, 'totalPrice', &quot;4372afbd-4edc-4b7a-a485-0bca0099a407&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
